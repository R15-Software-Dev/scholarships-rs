use leptos::prelude::*;

/// A custom-styled select input.
///
/// Takes a `Vec<String>` as the set of inputs. Only one value may be selected, which is indicated
/// by the `value` prop of the component.
#[component]
pub fn Select(
    #[prop(default = vec!["Test Value".into()])] value_list: Vec<String>,
    #[prop(default = RwSignal::new("".into()))] value: RwSignal<String>,
    #[prop(optional, into)] label: String,
    #[prop(default = RwSignal::new(false))] disabled: RwSignal<bool>,
) -> impl IntoView {
    view! {
        <div class="flex flex-1">
            <label class="flex flex-col flex-1">
                <span class="ml-1.5 mb-0 font-bold">{label}</span>
                <select
                    class="relative flex-1 border-2 m-1.5 mt-0 p-1.5 rounded-md
                        transition-border duration-150
                        border-red-700 bg-transparent
                        disabled:border-gray-600 disabled:pointer-events-none disabled:bg-gray-600/33"
                    on:change:target=move |e| {
                        value.set(e.target().value());
                    }
                    disabled = disabled >
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
