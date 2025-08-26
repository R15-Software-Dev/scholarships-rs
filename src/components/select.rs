use leptos::prelude::*;

/// A custom-styled select input.
///
/// Takes a `Vec<String>` as the set of inputs. Only one value may be selected, which is indicated
/// by the `value` prop of the component.
#[component]
pub fn Select(
    #[prop(default = vec!["Test Value".into()])] value_list: Vec<String>,
    #[prop(default = RwSignal::new("".into()))] value: RwSignal<String>,
    #[prop(default = RwSignal::new("".into()))] label: RwSignal<String>,
    #[prop(default = RwSignal::new(false))] disabled: RwSignal<bool>,
) -> impl IntoView {
    view! {
        <div>
            <label>
                {label}
                <select
                    class="relative border-2 m-1.5 p-1.5 rounded-md"
                    class=(["border-red-700", "bg-transparent"], move || !disabled.get())
                    class=(["border-gray-600", "pointer-events-none", "bg-gray-600/33"], move || disabled.get())
                    on:change:target=move |e| {
                        value.set(e.target().value());
                    }>
                    // This closure handles the display of the options.
                    {move || {
                        value_list.iter().map(
                            move |value| {
                                let display = value.to_owned();
                                view! { <option value={display}>{display.clone()}</option> }
                            }
                        ).collect::<Vec<_>>()
                    }}
                </select>
            </label>
        </div>
    }
}
