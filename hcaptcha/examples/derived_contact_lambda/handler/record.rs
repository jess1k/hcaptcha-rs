use crate::handler::error::LambdaContactError;
use crate::handler::ContactForm;
use tracing::instrument;

#[instrument(
    name = "Write record to database"
    skip(form)
    fields(email = %form.email)
)]
pub async fn write(form: &ContactForm) -> Result<(), LambdaContactError> {
    // Write the contact form data to dynamodb
    Ok(())
}
