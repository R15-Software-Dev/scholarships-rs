use leptos::prelude::*;

#[component]
pub fn Radio(
    #[prop()] checked: Signal<bool>,
    #[prop(into)] on_change: Callback<(), ()>,
    #[prop(into)] value: String,
    #[prop(default = RwSignal::new(false))] disabled: RwSignal<bool>,
) -> impl IntoView {
    view! {
        <label for=value class="flex items-center">
            <input
                // Using the "peer" class links this input with the div "peer" at the same level.
                // By using the "peer-checked" selector this div can toggle states based on this
                // input's state (checked/unchecked)
                class="
                relative peer shrink-0
                hidden"
                type="checkbox"
                id=value.clone()
                on:change=move |_| on_change.run(())
                prop:checked=checked
                disabled=disabled
            />
            // This div displays the box for the checkbox, and applies some styling to the child
            // span element.
            <div class="relative inline-block h-5 w-5 m-1 mr-2 rounded-full border border-gray-300 bg-white
            transition-bg duration-200 peer-checked:bg-red-700 peer-checked:border-red-700
            [&>span]:opacity-0 peer-checked:[&>span]:opacity-100
            peer-disabled:bg-gray-600/33 peer-disabled:border-gray-600
            peer-disabled:peer-checked:bg-gray-600">
                // This span is the check icon. It is centered within the div and has a border on the bottom
                // and right sides, which is then rotated to appear like a checkmark.
                <span class="absolute left-1/2 top-1/2 block h-2.5 w-2.5 -translate-x-1/2 -translate-y-1/2
                rotate-45 transform border-3 border-3 rounded-full bg-white
                border-white transition-opacity duration-150"></span>
            </div>
            <span>{value.clone()}</span>
        </label>
    }
}

#[component]
pub fn RadioList(
    #[prop(default = RwSignal::new(String::new()))] selected: RwSignal<String>,
    #[prop()] items: Vec<String>,
    #[prop(default = RwSignal::new(false))] disabled: RwSignal<bool>,
) -> impl IntoView {
    view! {
        // Checkbox elements go in here. Each one will have a derived signal
        // that checks if the name/value of that checkbox is in the selected items
        // vec. If so, display as selected. If not, clear that checkbox.

        // Somehow create a derived signal??

        <div>
            {items
                .into_iter()
                .map(|item| {
                    let is_checked = {
                        let item_name = item.clone();
                        move || { selected.get() == item_name }
                    };
                    let checked_signal = Signal::derive(is_checked);
                    // Check if this checkbox should be selected

                    view! {
                        <Radio
                            checked=checked_signal
                            on_change={
                                let item_name = item.clone();
                                move || selected.set(item_name.clone())
                            }
                            // The actual selected values are tracked by this element, not by the checkboxes themselves.
                            value=item.clone()
                            disabled=disabled
                        />
                    }
                })
                .collect::<Vec<_>>()}
        </div>
    }
}
