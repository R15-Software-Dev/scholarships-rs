use leptos::prelude::*;
use crate::components::toasts::toast::Toast;

#[derive(Clone, Debug)]
pub struct Toast {
    id: String,
    message: String,
}

impl Toast {
    pub fn new() -> Self {
        Toast {
            id: "".to_owned(),
            message: "".to_owned(),
        }
    }
    
    pub fn msg(mut self, msg: impl Into<String>) -> Self {
        self.message = msg.into();
        self
    }
    
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = id.into();
        self
    }
}

#[derive(Clone, Copy)]
pub struct ToastContext {
    messages: RwSignal<Vec<Toast>>,
}

impl ToastContext {
    pub fn new() -> Self {
        Self {
            messages: RwSignal::new(vec![]),
        }
    }
    
    pub fn toast(&mut self, toast: Toast) {
        self.messages.update(|list| list.push(toast));
    }
}

/// # Toast List Component
/// Displays a list of toasts. They must be queued using the [`ToastContext`].
#[component]
pub fn ToastList(
    children: Children,
) -> impl IntoView {
    // This context should be used wherever the application might need to use a toast.
    // That specific component should also be wrapped in a ToastList component, otherwise
    // this context won't exist.
    let context = use_context::<ToastContext>()
        .unwrap_or_else(provide_toast_context);
    
    view! {
        {children()}
        <div class="fixed top-1 left-1/2 right-1/2 -translate-x-1/2 min-w-lg">
            <For
                each=move || context.messages.get()
                key=|toast| toast.id.clone()
                let(Toast { id, message })
            >
                <Toast
                    message=message
                    title="Testing Toast"
                    on_close=move || {
                        context.messages.update(|list|
                            list.retain(|v| v.id != id)
                        );
                    }
                />
            </For>
        </div>
    }
}

fn provide_toast_context() -> ToastContext {
    let context = ToastContext::new();
    provide_context(context);
    context
}