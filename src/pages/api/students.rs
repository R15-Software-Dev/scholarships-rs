use leptos::prelude::*;
use leptos::server_fn::codec::Json;
use std::collections::HashMap;
use std::io::Write;
use std::process::Stdio;

#[cfg(feature = "ssr")]
mod imports {
    pub use super::super::MAIN_TABLE_NAME;
    pub use crate::common::ValueType;
    pub use crate::pages::api::tokens::validate_and_get_token_info;
    pub use crate::utils::server::*;
    pub use aws_sdk_dynamodb::error::ProvideErrorMetadata;
    pub use aws_sdk_dynamodb::types::AttributeValue;
    pub use leptos::logging::{debug_log, error};
    pub use leptos::serde_json;
    pub use std::collections::HashMap;
    pub use std::process::Command;
    pub use zip::write::SimpleFileOptions;
}

static PDF_TEMPLATE: &str = include_str!("../../../pdf_template.typ");

#[server(input = Json)]
pub async fn put_student_data(
    subject: String,
    data_type: String,
    data_map: HashMap<String, crate::common::ValueType>,
) -> Result<(), ServerFnError> {
    use imports::*;

    let client = create_dynamo_client().await;

    let mut data_map_attr = into_attr_map(data_map);
    data_map_attr.insert(
        "HK".into(),
        AttributeValue::S(format!("STUDENT#{}", subject)),
    );
    data_map_attr.insert("SK".into(), AttributeValue::S(data_type.to_uppercase()));

    debug_log!("Inserting this item: {:?}", data_map_attr);

    client
        .put_item()
        .table_name(MAIN_TABLE_NAME)
        .set_item(Some(data_map_attr))
        .send()
        .await
        .map(|_| ())
        .map_err(|err| {
            let msg = format!(
                "Couldn't put item into Dynamo: {}",
                err.message().unwrap_or("Unknown error occurred")
            );
            error!("{}", msg);
            ServerFnError::new(msg)
        })
}

#[server]
pub async fn get_student_data(
    subject: String,
    data_type: String,
) -> Result<HashMap<String, crate::common::ValueType>, ServerFnError> {
    use imports::*;

    let client = create_dynamo_client().await;

    client
        .query()
        .table_name(MAIN_TABLE_NAME)
        .key_condition_expression("HK = :hk AND SK = :sk")
        .expression_attribute_values(":hk", AttributeValue::S(format!("STUDENT#{}", subject)))
        .expression_attribute_values(":sk", AttributeValue::S(data_type.to_uppercase()))
        .send()
        .await
        .map(|output| {
            // Map the output to a HashMap<String, ValueType>
            let Some(list) = output.items else {
                return HashMap::new();
            };

            let mut map = HashMap::new();

            let _ = list.into_iter().flatten().for_each(|(k, v)| {
                map.insert(k, ValueType::from(&v));
            });

            map
        })
        .map_err(|err| {
            let msg = format!(
                "Couldn't get item from Dynamo: {}",
                err.message().unwrap_or("Unknown error occurred")
            );
            error!("{}", msg);
            ServerFnError::new(msg)
        })
}

/// Gets and flattens all of a student's information from the database, regardless of their defined
/// sort key.
///
/// Note: if sorted data from the `put_student_data` function contains the same fields, this
/// function will have unexpected behavior. For example, if a `first_name` field appears in multiple
/// locations in the database, all for this single student, they will overwrite each other in no
/// specific order.
#[server]
pub async fn get_all_student_data(
    subject: String,
) -> Result<HashMap<String, crate::common::ValueType>, ServerFnError> {
    use imports::*;

    let client = create_dynamo_client().await;

    client
        .query()
        .table_name(MAIN_TABLE_NAME)
        .key_condition_expression("HK = :hk")
        .expression_attribute_values(":hk", AttributeValue::S(format!("STUDENT#{}", subject)))
        .send()
        .await
        .map(|output| {
            let Some(items) = output.items else {
                return HashMap::new();
            };

            items
                .iter()
                .flatten()
                .map(|(k, v)| (k.clone(), ValueType::from(v)))
                .collect::<HashMap<String, ValueType>>()
        })
        .map_err(|err| {
            let msg = format!(
                "Couldn't get data from Dynamo: {}",
                err.message().unwrap_or("Unknown error occurred")
            );
            error!("{}", msg);
            ServerFnError::new(msg)
        })
}

#[server]
pub async fn provider_get_completed_students(
    access_token: String,
) -> Result<HashMap<String, HashMap<String, crate::common::ValueType>>, ServerFnError> {
    use imports::*;

    let claims =
        validate_and_get_token_info(access_token, "us-east-1_Lfjuy5zaM", "us-east-1").await?;
    if !claims.groups.contains(&"ScholarshipProviders".to_string()) {
        return Err(ServerFnError::new(
            "User is not in the ScholarshipProviders group",
        ));
    }

    get_completed_students().await
}

#[server]
pub async fn admin_get_completed_students(
    access_token: String,
) -> Result<HashMap<String, HashMap<String, crate::common::ValueType>>, ServerFnError> {
    use imports::*;

    let _ = validate_and_get_token_info(access_token, "us-east-1_rvCU4Xy4j", "us-east-1").await?;

    get_completed_students().await
}

#[server]
async fn get_completed_students()
-> Result<HashMap<String, HashMap<String, crate::common::ValueType>>, ServerFnError> {
    use imports::*;

    // We want to get all student information. The requirements are that the students have completed
    // the demographics form - everything else may bed from this.
    // The easiest way is to get all the information and filter on this side, instead of bookkeeping
    // on the database's side.

    let client = create_dynamo_client().await;

    let response = client
        .scan()
        .table_name(MAIN_TABLE_NAME)
        .send()
        .await
        .map_err(|err| {
            let msg = err.message().unwrap_or("Unknown error occurred");
            error!("{}", msg);
            ServerFnError::new(msg)
        })?;

    let Some(items) = response.items else {
        return Ok(HashMap::new());
    };

    let mut output = HashMap::<String, HashMap<String, ValueType>>::new();

    items.into_iter().for_each(|mut form_info| {
        let student_id_full = form_info
            .get("HK")
            .map(|v| v.as_s().cloned().unwrap_or_default())
            .unwrap_or_default()
            .to_owned();

        let student_id = student_id_full.split("STUDENT#").collect::<String>();

        form_info.remove("HK");
        form_info.remove("SK");

        let form_info_convert = form_info
            .into_iter()
            .map(|(k, v)| (k, ValueType::from(&v)))
            .collect::<HashMap<String, ValueType>>();

        // We want to insert all the remaining information into the output map
        output
            .entry(student_id)
            .and_modify(|v| {
                v.extend(form_info_convert.clone());
            })
            .or_insert(form_info_convert);
    });

    // Don't love this, but it does verify that the student has completed the demographics form
    let output = output
        .into_iter()
        .filter_map(|(k, student_info)| {
            student_info
                .get("first_name")
                .and_then(|_| Some((k, student_info.clone())))
        })
        .collect();

    Ok(output)
}

/// # Get File by Key API
/// This function gets a file from the corresponding S3 file key. If the subject found from the
/// `access_token` does not match the owner of the requested file, the requesting user must have
/// provider-level access or higher.
///
/// ## Possible errors
/// Please check the [`validate_and_get_token_info`] function for all token parsing errors. Otherwise,
/// if the requesting user does not have the correct permissions, it will return a message indicating
/// such. If the file is not found, an error containing "File not found" will be returned.
#[server]
pub async fn get_file_by_key(
    access_token: String,
    file_key: String,
) -> Result<Vec<u8>, ServerFnError> {
    use imports::*;

    let user_claims =
        validate_and_get_token_info(access_token, "us-east-1_Lfjuy5zaM", "us-east-1").await?;

    // Check if the user's subject is contained in the file_key (since all files are keyed by the
    // user's subject)
    if !file_key.contains(&user_claims.subject) {
        // Check that the user is a provider.
        if !user_claims
            .groups
            .contains(&"ScholarshipProviders".to_string())
        {
            return Err(ServerFnError::new(
                "Access denied: user is not the student or a provider",
            ));
        }
    }

    // We now just need to get the actual file.
    let client = aws_sdk_s3::Client::new(&create_aws_config().await);

    let result = client
        .get_object()
        .key(file_key)
        .send()
        .await
        .map_err(|e| {
            let msg = e.message().unwrap_or("Unknown error occurred");
            error!("{}", msg);
            ServerFnError::new(msg)
        })?;

    let bytes = result.body.collect().await?;

    Ok(bytes.to_vec())
}

/// # Get Student File Names API
/// Gets a list of file names given a student's ID and the corresponding input ID from the forms.
/// These file names can be used to construct a file key for S3.
///
/// This API is only accessible by provider-level users.
#[server]
pub async fn get_student_files(
    access_token: String,
    student_id: String,
    form_name: String,
    question_id: String,
    file_name_postfix: String,
) -> Result<(String, Vec<u8>), ServerFnError> {
    use imports::*;

    match validate_and_get_token_info(access_token.clone(), "us-east-1_Lfjuy5zaM", "us-east-1")
        .await
    {
        Ok(claims) => {
            if !claims.groups.contains(&"ScholarshipProviders".to_string()) {
                return Err(ServerFnError::new("Access denied: user is not a provider"));
            }
        }
        Err(_) => {
            let _ = validate_and_get_token_info(access_token, "us-east-1_rvCU4Xy4j", "us-east-1")
                .await?;
        }
    };

    let client = create_dynamo_client().await;

    let demographics_res = client
        .query()
        .table_name(MAIN_TABLE_NAME)
        .expression_attribute_values(":hk", AttributeValue::S(format!("STUDENT#{student_id}")))
        .expression_attribute_values(":sk", AttributeValue::S("DEMOGRAPHICS".to_string()))
        .key_condition_expression("HK = :hk and SK = :sk")
        .send()
        .await
        .map_err(|e| {
            let msg = e.message().unwrap_or("Unknown error occurred");
            error!("{}", msg);
            ServerFnError::new(msg)
        })?;

    let student_demographics = demographics_res
        .items
        .unwrap_or_default()
        .into_iter()
        .next()
        .ok_or(ServerFnError::new(
            "Failed to find student demographic information.",
        ))?;

    let first_name = student_demographics
        .get("first_name")
        .and_then(|v| v.as_s().ok().cloned())
        .unwrap_or_default();

    let last_name = student_demographics
        .get("last_name")
        .and_then(|v| v.as_s().ok().cloned())
        .unwrap_or_default();

    let file_keys = client
        .query()
        .table_name(MAIN_TABLE_NAME)
        .expression_attribute_values(":hk", AttributeValue::S(format!("STUDENT#{student_id}")))
        .expression_attribute_values(
            ":sk",
            AttributeValue::S(format!("FILE#{form_name}#{question_id}")),
        )
        .key_condition_expression("HK = :hk and begins_with(SK, :sk)")
        .send()
        .await
        .map_err(|e| {
            let msg = e.message().unwrap_or("Unknown error occurred");
            error!("{}", msg);
            ServerFnError::new(msg)
        })
        .map(|output| {
            let Some(items) = output.items else {
                return Vec::new();
            };

            items
                .into_iter()
                .filter_map(|item| item.get("file_key")?.as_s().ok().cloned())
                .collect::<Vec<String>>()
        })?;

    debug_log!("Await student file futures...");
    let futures: Vec<_> = file_keys
        .into_iter()
        .map(|file_key| async move {
            debug_log!("Requesting file from API with key {file_key}");
            // Get all files from S3. This is a batch operation.
            let s3_client = aws_sdk_s3::Client::new(&create_aws_config().await);
            let file_output = s3_client
                .get_object()
                .bucket("leptos-scholarships")
                .key(&file_key)
                .send()
                .await
                .ok()?;

            let bytes = file_output.body.collect().await.ok()?.to_vec();

            Some((file_key, bytes))
        })
        .collect();

    let results = futures::future::join_all(futures)
        .await
        .into_iter()
        .flatten()
        .collect::<Vec<(String, Vec<u8>)>>();

    let cur = std::io::Cursor::new(Vec::new());
    let mut writer = zip::ZipWriter::new(cur);

    debug_log!("Writing {} files to zip file...", results.len());
    for (key, file_bytes) in results {
        let file_name = key
            .split('/')
            .last()
            .ok_or(ServerFnError::new("Failed to parse file name"))?
            .split(".")
            .next()
            .unwrap_or_default();
        let file_ext = key
            .split('.')
            .last()
            .ok_or(ServerFnError::new("Failed to parse file extension"))?;

        // Construct the real file name. This name should be under 150 characters.
        let temp = format!("{first_name} {last_name} - {file_name}");
        let file_name_no_ext = temp.chars().take(150).collect::<String>();
        let file_name_complete = format!("{file_name_no_ext}.{file_ext}");

        writer
            .start_file(file_name_complete, SimpleFileOptions::default())
            .map_err(ServerFnError::new)?;
        writer.write_all(&file_bytes).map_err(ServerFnError::new)?;
    }

    debug_log!("Getting finished file");
    let finished = writer.finish().map_err(ServerFnError::new)?;

    let file_name_hint = format!("{first_name}{last_name}_{file_name_postfix}.zip");

    Ok((file_name_hint, finished.into_inner()))
}

/// # Get All Input Files API
/// Gets all files that have been uploaded to the given input. For example, getting all the FAFSA
/// files that were uploaded to the FAFSA file input on the Financial Info page.
///
/// Returns a `HashMap` keyed by the submitting user's ID, with a value of all files that were
/// uploaded by that user.
///
/// This is only usable by users with provider-level access.
#[server]
async fn get_all_input_files(
    form_name: String,
    input_name: String,
) -> Result<HashMap<String, Vec<String>>, ServerFnError> {
    use imports::*;

    let client = create_dynamo_client().await;

    let res = client
        .scan()
        .table_name(MAIN_TABLE_NAME)
        .filter_expression("begins_with(SK, :sk)")
        .expression_attribute_values(
            ":sk",
            AttributeValue::S(format!("FILE#{form_name}#{input_name}")),
        )
        .send()
        .await
        .map_err(|e| {
            let msg = e.message().unwrap_or("Unknown error occurred");
            error!("{}", msg);
            ServerFnError::new(msg)
        })?;

    let mut result_map = HashMap::<String, Vec<String>>::new();
    res.items.unwrap_or_default().iter().for_each(|item| {
        let id = item
            .get("HK")
            .map(|v| v.as_s().cloned().unwrap_or_default())
            .unwrap_or_default()
            .split("STUDENT#")
            .collect::<String>();

        let file_key = item
            .get("file_key")
            .map(|v| v.as_s().cloned().unwrap_or_default())
            .unwrap_or_default();

        result_map
            .entry(id)
            .and_modify(|v| v.push(file_key.clone()))
            .or_insert(vec![file_key]);
    });

    Ok(result_map)
}

#[server]
pub async fn admin_get_all_input_files(
    access_token: String,
    form_name: String,
    input_name: String,
) -> Result<HashMap<String, Vec<String>>, ServerFnError> {
    use imports::*;

    let _ = validate_and_get_token_info(access_token, "us-east-1_rvCU4Xy4j", "us-east-1").await?;

    get_all_input_files(form_name, input_name).await
}

#[server]
pub async fn provider_get_all_input_files(
    access_token: String,
    form_name: String,
    input_name: String,
) -> Result<HashMap<String, Vec<String>>, ServerFnError> {
    use imports::*;

    let claims =
        validate_and_get_token_info(access_token, "us-east-1_Lfjuy5zaM", "us-east-1").await?;
    if !claims.groups.contains(&"ScholarshipProviders".to_string()) {
        return Err(ServerFnError::new("Access denied: user is not a provider"));
    }

    get_all_input_files(form_name, input_name).await
}

#[server]
pub async fn get_student_pdf(student_id: String) -> Result<(String, Vec<u8>), ServerFnError> {
    use imports::*;

    // NOTE: this API requires that the server has the typst-cli available ON PATH.

    let student_info_str = get_student_info_json(student_id.clone()).await?;
    let student_info = get_student_data(student_id, "DEMOGRAPHICS".to_string()).await?;

    let first_name = student_info
        .get("first_name")
        .map(|v| v.to_string())
        .unwrap_or_default();

    let last_name = student_info
        .get("last_name")
        .map(|v| v.to_string())
        .unwrap_or_default();

    let typst_string = student_info_str
        .replace("{", "(")
        .replace("}", ")")
        .replace("[", "(")
        .replace("]", ")")
        // Ensures trailing commas for lists of dictionaries
        .replace("))", "),)");

    debug_log!("New string: {}", typst_string);

    let pdf_string = format!("#let student = {typst_string}\n{PDF_TEMPLATE}");
    debug_log!("PDF string: {}", pdf_string);
    let mut child = Command::new("typst")
        .arg("compile")
        .arg("-")
        .arg("-")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    {
        let stdin = child.stdin.as_mut().unwrap();
        stdin.write_all(pdf_string.as_bytes())?;
        stdin.flush()?;
    }

    let output = child.wait_with_output()?;

    if !output.status.success() {
        let msg = String::from_utf8_lossy(&output.stderr).to_string();
        error!("Failed to generate PDF: {}", msg);
        Err(ServerFnError::new(format!("Failed to generate PDF: {msg}")))
    } else {
        let bytes = output.stdout;
        let file_name = format!("{first_name}{last_name}_Application.pdf");
        Ok((file_name, bytes))
    }
}

#[server]
pub async fn get_student_info_json(student_id: String) -> Result<String, ServerFnError> {
    use imports::*;

    let client = create_dynamo_client().await;

    let response = client
        .query()
        .table_name(MAIN_TABLE_NAME)
        .expression_attribute_values(":hk", AttributeValue::S(format!("STUDENT#{student_id}")))
        .key_condition_expression("HK = :hk")
        .send()
        .await
        .map_err(|e| {
            let msg = e.message().unwrap_or("Unknown error occurred");
            error!("{}", msg);
            ServerFnError::new(msg)
        })?;

    let work_exp = client
        .query()
        .table_name(MAIN_TABLE_NAME)
        .expression_attribute_values(":hk", AttributeValue::S(format!("STUDENT#{student_id}")))
        .expression_attribute_values(":sk", AttributeValue::S("WORKEXP".to_string()))
        .key_condition_expression("HK = :hk AND SK = :sk")
        .send()
        .await
        .map_err(|e| {
            let msg = e.message().unwrap_or("Unknown error occurred");
            error!("{}", msg);
            ServerFnError::new(msg)
        })
        .map(|output| {
            let items = output.items.unwrap_or_default();
            debug_log!("Number of items: {}", items.len());
            items
                .into_iter()
                .next()
                .unwrap_or_default()
                .get("extracurricular")
                .cloned()
                .unwrap_or(AttributeValue::S("".to_string()))
        })?;

    let extracurricular = client
        .query()
        .table_name(MAIN_TABLE_NAME)
        .expression_attribute_values(":hk", AttributeValue::S(format!("STUDENT#{student_id}")))
        .expression_attribute_values(":sk", AttributeValue::S("DEMOGRAPHICS".to_string()))
        .key_condition_expression("HK = :hk AND SK = :sk")
        .send()
        .await
        .map_err(|e| {
            let msg = e.message().unwrap_or("Unknown error occurred");
            error!("{}", msg);
            ServerFnError::new(msg)
        })
        .map(|output| {
            let items = output.items.unwrap_or_default();
            debug_log!("Number of items: {}", items.len());
            items
                .into_iter()
                .next()
                .unwrap_or_default()
                .get("extracurricular")
                .cloned()
                .unwrap_or(AttributeValue::S("".to_string()))
        })?;

    let Some(items) = response.items else {
        return Err(ServerFnError::new("Failed to find student information."));
    };

    // Join items together into a single object.
    let mut data = items.into_iter().fold(HashMap::new(), |mut map, item| {
        item.into_iter().for_each(|(k, v)| {
            map.insert(k, v);
        });
        map
    });

    data.insert("work_experience".to_string(), work_exp);
    data.insert("extracurricular".to_string(), extracurricular);

    let json_value: serde_json::Value = serde_dynamo::from_item(data)?;
    let json_string = serde_json::to_string(&json_value)?;
    println!("{}", json_string);

    Ok(json_string)
}
