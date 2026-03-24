use super::*;

impl BootstrapGrants {
    pub fn new(reader_emails: Vec<user::Email>, operator_emails: Vec<user::Email>) -> Self {
        Self {
            reader_emails,
            operator_emails,
        }
    }

    pub fn configured_emails(&self) -> Vec<user::Email> {
        let mut emails = self.reader_emails.clone();
        for email in &self.operator_emails {
            if !emails.contains(email) {
                emails.push(email.clone());
            }
        }
        emails
    }

    fn capabilities_for(&self, email: &user::Email) -> Vec<sensitive::AccessCapability> {
        let mut capabilities = Vec::new();
        if self.reader_emails.contains(email) || self.operator_emails.contains(email) {
            capabilities.push(sensitive::AccessCapability::AuthorizedRecordRead);
        }
        if self.operator_emails.contains(email) {
            capabilities.push(sensitive::AccessCapability::TokenStatusRead);
            capabilities.push(sensitive::AccessCapability::AccessAuditRead);
        }
        super::snapshot::sorted_capabilities(capabilities)
    }
}

impl Service {
    pub async fn reconcile_bootstrap_grants_for_user(
        &self,
        user_id: user::Id,
        email: &user::Email,
    ) -> Result<Vec<sensitive::AccessCapability>> {
        let capabilities = self.bootstrap.capabilities_for(email);
        if !capabilities.is_empty() {
            self.repo
                .upsert_access_grants(&user_id, &capabilities, self.clock.now())
                .await?;
        }
        Ok(capabilities)
    }
}
