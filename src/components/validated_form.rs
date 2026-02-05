use leptos::logging::log;
use leptos::prelude::*;
use leptos::web_sys::SubmitEvent;
use crate::components::{ActionButton, Header};

/// Indicates the current state of an input.
#[derive(Default, Debug, Clone)]
pub enum ValidationState {
    /// A valid input.
    #[default]
    Valid,
    /// An invalid input with an error message.
    Invalid(String)
}

/// A struct that contains all information required for a form to make real-time validation
/// decisions, including display of error messages.
#[derive(Debug, Clone)]
pub struct InputState {
    /// The name of the input. This is used to identify the input in the form.
    pub input_name: String,
    /// Indicates the current state of the input.
    error: Signal<ValidationState>,
    /// Indicates whether the form has requested the error state. This means that the input
    /// should show their error appearances immediately.
    form_requested_validation: RwSignal<bool>,
}

impl InputState {
    /// Creates a new [`InputState`] with the given error and form requested validation signals.
    pub fn new(
        name: impl Into<String>,
        error: impl Into<Signal<ValidationState>>,
        form_requested_errors: RwSignal<bool>
    ) -> Self {
        Self {
            input_name: name.into(),
            error: error.into(),
            form_requested_validation: form_requested_errors
        }
    }

    pub fn error(&self) -> Signal<ValidationState> {
        self.error.clone()
    }
}

/// A list of [`InputState`]s.
pub type ValidatorList = Vec<RwSignal<InputState>>;

/// A struct that keeps track of all components that must be validated.
/// All of our custom-made components must be placed within an area that contains context with
/// this struct.
#[derive(Debug, Clone)]
pub struct FormValidationRegistry {
    pub validators: RwSignal<ValidatorList>
}

/// Gets the [`FormValidationRegistry`] context if it exists.
pub fn use_validation_context() -> Option<FormValidationRegistry> {
    use_context::<FormValidationRegistry>()
}

/// # Validated Form Component
///
/// A simple HTML `<form>` wrapper that executes validation logic using a [`FormValidationRegistry`].
/// It will ensure that any invalid inputs are caught before submission. All input components that
/// are used within this form should call `use_context::<FormValidationRegistry>()` to register
/// themselves with the form.
///
/// Example usage in the view macro:
/// ```
/// view! {
///     <ValidatedForm 
///         on_submit=Callback::new(move || {/* something here */})
///         title="Example Title"
///         description="Example description."
///     >
///         {/* As many input components as you want */}
///     </ValidatedForm>
/// }
/// ```
///
/// Example input component implementation:
/// ```
/// fn SomeComponent() -> impl IntoView {
///     // Get FormValidationContext
///     let context = use_context::<FormValidationRegistry>()
///         .expect("Couldn't find FormValidationRegsitry context");
///
///     // Register with the form
///     let error = RwSignal::new(ValidationState::Valid);
///     let form_requested_errors = RwSignal::new(false);
///     let state = InputState::new(
///         "the_input_name",
///         error.clone(),
///         form_requested_errors.clone()
///     );
///     context.validators.update(|list| list.push(RwSignal::new(state)));
///
///     /* Other input view logic */
/// }
/// ```
#[component]
pub fn ValidatedForm(
    children: Children,
    /// A callback that runs when the form successfully validates.
    #[prop(into)] on_submit: Callback<()>,
    /// The title of the included header.
    #[prop(into)] title: Signal<String>,
    /// The description for the included header.
    #[prop(optional, into)] description: Signal<String>,
) -> impl IntoView {
    let validators = RwSignal::new(vec![]);

    // Provide context to all children nested within this component. They will be able to access
    // this no matter how nested they are.
    provide_context(FormValidationRegistry {validators});

    let can_submit = Memo::new(move |_| {
        log!("Checking {} inputs...", validators.get().len());
        validators
            .with(|list| {
                list.iter()
                    // TODO all() short circuits on the first false
                    .fold(true, |acc, v| {
                        v.update(|validator| validator.form_requested_validation.set(true));
                        matches!(v.get().error().get(), ValidationState::Valid) && acc
                    })
                    // .all(|v| {
                    //     // Not sure how this will work reactively.
                    //     let result = matches!(v.get().error().get(), ValidationState::Valid);
                    //     v.update(|validator| validator.form_requested_validation.set(true));
                    //     result
                    // })
            })
    });

    let submit_success_event = move |e: SubmitEvent| {
        e.prevent_default();

        if !can_submit.get() {
            log!("Form failed to submit, there are errors in form inputs.");
            return;
        }

        log!("Form validation successful.");
        on_submit.run(());
    };

    view! {
        <form class="flex flex-col gap-2 py-7" novalidate on:submit=submit_success_event>
            <Header title=title description=description />
            {children()}
            <ActionButton
                on:click=move |_| {
                    log!("{:?}", can_submit.get());
                }
                button_type="submit"
            >
                "Submit"
            </ActionButton>
        </form>
    }
}
