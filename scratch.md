Forms should be built for this application with a series of JSON-type files.
Each form will have a series of sections, and each section will have a series of
questions. Each question has a type and a label - question styles may come later.
It may be possible for us to call out specific question types without needing
to parse the types separately, but that feels not-really-worth-it.

Example:

```
{
    "SectionDefinitions": [
        "SectionTitle": [
            "text": {
                "label": "First name question!",
                "placeholder": "John",
            },
            "text": {
                "label": "Last name question!",
                "placeholder": "Smith",
            }
        ],
        "Section Two": [
            "text": {
                "label": "Section two testing question",
                "placeholder": "Some example here",
            },
            "select": {
                "label": "This is a dropdown element!",
                "placeholder": "option1",
                "possibleValues": [
                    "option1": "something",
                    "option2": "another something",
                ]
            }
        ]
    ]
}
```

...and so on. Here's a likely set of structures to use:

```
struct FormDefinitionDoc {
    Vec<Section> section_defs;
    // and any extra values that should be applied
    // across the entire form, like settings.
}

struct Section {
    Vec<QuestionComponent> questions;
    // each section may have a series of settings including how to submit,
    // when to submit, and where to submit to at the very least.
}

struct QuestionComponent {
    label: String,
    placeholder: String,
    possible_values: HashMap<str, str>,
    // there are definitely values that i'll find while creating this structure.
}
```

Types of questions that are required, starting with the providers.

Providers must be able to:

 - Log into the website using a username and password (handled by AWS)
 - Create and manage multiple scholarships.
 - Input scholarship information for each scholarship.
 - View students that have applied.
 - View student application information (should just be labels??)
 - Buttons are a given. Duh.

Students must be able to:

 - Log in using their Region 15 email (handled by AWS)
 - Create and manage their general application.
 - Create and manage applications specific to a single scholarship.
 - View their acceptance status (need more confirmation)

Administrators must be able to:

 - Log in using their Region 15 email (handled by AWS)
 - Manage student scholarship applications.
 - Manage scholarship provider account information.
 - Create/manage scholarship information.
 - Download student applications if needed.
 - Statistics would be nice, but are likely a future thing.

Right off the bad, I think I can see a few types of buttons and text inputs,
along with dropdowns, checkboxes, our custom `MultiEntry` element from last
time. That means no new options, but it does mean that the `FormDefinition`
needs to be able to keep track of all this information.

These are some example `QuestionComponent` objects:

Text fields:

```
{
    question_type: "TEXT" | "EMAIL" | "NUMBER",
    label: "The question label",
    name: "input_name[jsdoc]",
    placeholder: "An example input"
}
```

Dropdowns/checkboxes/radios:

```
{
    question_type: "DROPDOWN" | "CHECKBOX" | "RADIO",
    label: "The question label",
    name: "input_name[jsdoc]",
    placeholder: "The pre-selected option value",
    options: [
        "optionValue": "Option Text"
    ]
}
```

`MultiEntry`:

```
{
    question_type: "MULTI",
    label: "The question label",
    name: "input_name[jsdoc]"
}
```

The real question here: how can we store this information in a database? And
this means specifically within DynamoDB, because this is not an easy structure
to store in there.
That's going to be the hardest apart and I'll likely involve the AI in that
planning.
