To track errors and force form input validation, we're going to create a custom `ValidatedForm` component.

This component will do two things:
 - Provide a submit button (that is disabled if the form is invalid, still thinking about that)
 - Define a validation method that will be called on submit that's updated on a per-input basis
   (this likely looks like a `Vec<RwSignal<ValidationState>>`)

In order for this to work, the form will `provide_context` to the `Vec`, and then it will check this
vector on submit. It doesn't worry about the states inbetween, only during the submission logic.

The inputs start off with a default state, which will be defined by that input's validation method.
This means that every input will run the validation method on mount and on each change. The first time the
function is called should be during mount, which will set the default state. This does mean that any inputs
that are marked as required will always be invalid on mount, and so we need to determine when these 
error messages should be shown and updated.

I think they should be shown in these situations:
 - Text inputs: on blur - we don't want to scream at users as they type.
 - Selectable inputs: on change and form submit - only show the message if the value is invalid, and not at mount.

To check values for the selectable inputs at submit time, we'll probably need to provide a refresh method
that can be called on submit. This method must somehow be callable from the outside of the component and
will show the error messages. We'd likely store all this in a struct of some kind, which could contain an
`error` and a `show_message` signal.

We already know that we can send any `Signal` to another component, so we can just send the `show_message`
signal to the parent component without any issues. We'll just use that signal to show the error messages 
in the component as well. Usually we don't want to edit too many signals outside their local context,
but this is a special case – documentation is therefore incredibly important! Remember that we may also
edit the `show_message` signal within its local context as well, so we need to make sure there are no cyclical
dependencies. Alternatively, we could not edit the `show_message` signal at all locally and just use it
to trigger side effects that show messages. Have to think about this more before implementation.

Here's a rough sketch of it all put together:
```rust
enum ValidationState {
   /// A valid input!
   Valid,
   /// An invalid input with an error message.
   Invalid(String)
}

// We'll store this in a Vec<RwSignal<InputState>>
struct InputState {
   /// Determines if there is an error and provides an error message.
   error: Signal<ValidationState>,
   /// Determines if the form has requested to show error messages.
   show_message: RwSignal<bool>
}

// Define a new type for ease of use.
type ValidationVec = Vec<RwSignal<InputState>>;

// Parent form component
#[component]
fn Form() -> impl IntoView {
   let validation_states: ValidationVec = Vec::new();
   
   provide_context(validation_states);
   
   /* All other form logic */
}

fn validation_function(input: String) -> ValidationState {
   // Validation logic is specific to each input type.
}

// Child component
#[component]
fn AnyInput() -> impl IntoView {
   let validation_states = use_context::<ValidationVec>().expect("Couldn't find form validation context");
   let current_input = RwSignal::new(String::new());  // Obviously, this would have a default value.
   let error_signal = RwSignal::new(validation_function(current_input.get()));
   let show_errors = RwSignal::new(false);  // Don't show them on mount!
   
   // See below for InputState definition
   let error_state = InputState {
      error: error_signal.clone().into(),
      show_message: show_errors.clone()
   };
   
   validation_states.push(RwSignal::new())
   
   /* All other input logic */
}
```
