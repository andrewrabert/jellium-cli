use clap::Subcommand;
use uuid::Uuid;

#[derive(Subcommand)]
pub enum PersonsCommand {
    /// List all persons
    List {
        /// Maximum number of records to return
        #[arg(long)]
        limit: Option<i32>,
        /// Search term
        #[arg(long)]
        search_term: Option<String>,
        /// Additional fields to return (comma-delimited)
        #[arg(long, value_delimiter = ',')]
        fields: Option<Vec<jellyfin_api::types::ItemFields>>,
        /// Filter by items related to this item ID
        #[arg(long)]
        appears_in_item_id: Option<Uuid>,
        /// Filter by person types (comma-delimited)
        #[arg(long, value_delimiter = ',')]
        person_types: Option<Vec<String>>,
        /// Exclude person types (comma-delimited)
        #[arg(long, value_delimiter = ',')]
        exclude_person_types: Option<Vec<String>>,
        /// Filter by favorite status
        #[arg(long)]
        is_favorite: Option<bool>,
        /// User ID (defaults to session user)
        #[arg(long)]
        user_id: Option<Uuid>,
    },
    /// Get a person by name
    Get {
        /// The person name
        name: String,
        /// User ID (defaults to session user)
        #[arg(long)]
        user_id: Option<Uuid>,
    },
}

pub async fn execute(
    client: &jellyfin_api::Client,
    user_id: &Uuid,
    command: &PersonsCommand,
) -> Result<(), Box<dyn std::error::Error>> {
    match command {
        PersonsCommand::List {
            limit,
            search_term,
            fields,
            appears_in_item_id,
            person_types,
            exclude_person_types,
            is_favorite,
            user_id: uid,
        } => {
            let effective_uid = uid.as_ref().unwrap_or(user_id);
            let result = client
                .get_persons(&jellyfin_api::query::GetPersons {
                    appears_in_item_id: appears_in_item_id.as_ref(),
                    exclude_person_types: exclude_person_types.as_ref(),
                    fields: fields.as_ref(),
                    is_favorite: *is_favorite,
                    limit: *limit,
                    person_types: person_types.as_ref(),
                    search_term: search_term.as_deref(),
                    user_id: Some(effective_uid),
                    ..Default::default()
                })
                .await?;
            crate::output::print_json(&result)?;
        }
        PersonsCommand::Get { name, user_id: uid } => {
            let effective_uid = uid.as_ref().unwrap_or(user_id);
            let result = client.get_person(name, Some(effective_uid)).await?;
            crate::output::print_json(&result)?;
        }
    }
    Ok(())
}
