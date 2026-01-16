use crate::components::{Banner, DashboardButton, DateList, Loading};
use leptos::prelude::*;
use leptos_oidc::{AuthLoaded, Authenticated};
use crate::pages::api::get_important_dates;
use crate::pages::UnauthenticatedPage;

#[component]
pub fn ProviderPortal() -> impl IntoView {
    let dates_resource = OnceResource::new(async move {
        get_important_dates().await
    });

    let dates_list = RwSignal::new(Vec::new());

    Effect::new(move || {
        if let Some(Ok(mut dates)) = dates_resource.get() {
            dates.sort_by_key(|info| info.date.get_status());
            dates_list.set(dates);
        }
    });
    
    view! {
        <Banner title="Provider Dashboard" logo="PHS_Stacked_Acronym.png" path="/" />
        <AuthLoaded fallback=Loading>
            <Authenticated unauthenticated=UnauthenticatedPage>
                <div class="mx-auto px-6">
                    <div class="container mx-auto px-6 py-8 space-y-8">
                        <div class="grid grid-cols-1 lg:grid-cols-2 gap-8">
                            <section class="space-y-4">
                                <DashboardButton
                                    title="Profile"
                                    description="Edit user profile"
                                    icon="/Person_Black.png"
                                    path="/providers/profile"
                                />
                                <DashboardButton
                                    title="Create Scholarship"
                                    description="Navigate to Scholarship Creation Page"
                                    icon="/Create_Black.png"
                                    path="/providers/scholarships"
                                />
                                <DashboardButton
                                    title="Applicants"
                                    description="View Scholarship Applicants"
                                    icon="/Form_Black.png"
                                    path="/"
                                />
                            </section>
                            <section class="space-y-4">
                                <div class="rounded-lg shadow-lg/25 overflow-hidden">
                                    <div class="bg-red-900 text-white px-4 py-3 shadow-lg">
                                        <h3 class="text-white font-bold">Important Dates</h3>
                                    </div>
                                    <div class="p-2">
                                        <div class="flex flex-col p-3 gap-3">
                                            <DateList dates=dates_list />
                                        </div>
                                    </div>
                                </div>
                            </section>
                        </div>
                    </div>
                </div>
            </Authenticated>
        </AuthLoaded>
    }
}
