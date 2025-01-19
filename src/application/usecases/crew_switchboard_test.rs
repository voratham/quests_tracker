#[cfg(test)]

mod tests {

    use std::sync::Arc;

    use anyhow::Ok;
    use chrono::{TimeZone, Utc};

    use crate::{
        application::usecases::crew_switchboard::CrewSwitchboardUseCase,
        domain::{
            entities::quests::QuestEntity,
            repositories::{
                crew_switchboard::MockCrewSwitchboardRepository,
                quest_viewing::MockQuestViewingRepository,
            },
            value_objects::quest_statuses::QuestStatuses,
        },
    };

    #[tokio::test]
    async fn test_join_success() {
        let mut mock_crew_switchboard_repo = MockCrewSwitchboardRepository::new();
        let mut mock_quest_viewing_repo = MockQuestViewingRepository::new();

        mock_quest_viewing_repo
            .expect_adventurers_counting_by_quest_id()
            .returning(|_| Box::pin(async { Ok(2) }));

        mock_quest_viewing_repo
            .expect_view_details()
            .returning(|_| {
                Box::pin(async {
                    Ok(QuestEntity {
                        id: 1,
                        name: "test quest 1".to_string(),
                        description: Some("test quest description".to_string()),
                        status: QuestStatuses::Open.to_string(),
                        guild_commander_id: 1,
                        created_at: Utc
                            .with_ymd_and_hms(2025, 1, 1, 0, 0, 0)
                            .unwrap()
                            .naive_utc(),
                        updated_at: Utc
                            .with_ymd_and_hms(2025, 1, 1, 0, 0, 0)
                            .unwrap()
                            .naive_utc(),
                    })
                })
            });

        mock_crew_switchboard_repo
            .expect_join()
            .returning(|_| Box::pin(async { Ok(()) }));

        let use_case = CrewSwitchboardUseCase::new(
            Arc::new(mock_crew_switchboard_repo),
            Arc::new(mock_quest_viewing_repo),
        );

        let result = use_case.join(1, 1).await;

        assert!(result.is_ok())
    }

    #[tokio::test]
    async fn test_join_fails_when_quest_is_not_open() {
        let mut mock_crew_switchboard_repo = MockCrewSwitchboardRepository::new();
        let mut mock_quest_viewing_repo = MockQuestViewingRepository::new();

        mock_quest_viewing_repo
            .expect_adventurers_counting_by_quest_id()
            .returning(|_| Box::pin(async { Ok(2) }));

        mock_quest_viewing_repo
            .expect_view_details()
            .returning(|_| {
                Box::pin(async {
                    Ok(QuestEntity {
                        id: 1,
                        name: "test quest 1".to_string(),
                        description: Some("test quest description".to_string()),
                        status: QuestStatuses::InJourney.to_string(),
                        guild_commander_id: 1,
                        created_at: Utc
                            .with_ymd_and_hms(2025, 1, 1, 0, 0, 0)
                            .unwrap()
                            .naive_utc(),
                        updated_at: Utc
                            .with_ymd_and_hms(2025, 1, 1, 0, 0, 0)
                            .unwrap()
                            .naive_utc(),
                    })
                })
            });

        mock_crew_switchboard_repo
            .expect_join()
            .returning(|_| Box::pin(async { Ok(()) }));

        let use_case = CrewSwitchboardUseCase::new(
            Arc::new(mock_crew_switchboard_repo),
            Arc::new(mock_quest_viewing_repo),
        );

        let result = use_case.join(1, 1).await;

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "The quest not joinable")
    }

    #[tokio::test]
    async fn test_join_fails_when_quest_has_adventurers_full() {
        let mut mock_crew_switchboard_repo = MockCrewSwitchboardRepository::new();
        let mut mock_quest_viewing_repo = MockQuestViewingRepository::new();

        mock_quest_viewing_repo
            .expect_adventurers_counting_by_quest_id()
            .returning(|_| Box::pin(async { Ok(4) }));

        mock_quest_viewing_repo
            .expect_view_details()
            .returning(|_| {
                Box::pin(async {
                    Ok(QuestEntity {
                        id: 1,
                        name: "test quest 1".to_string(),
                        description: Some("test quest description".to_string()),
                        status: QuestStatuses::Open.to_string(),
                        guild_commander_id: 1,
                        created_at: Utc
                            .with_ymd_and_hms(2025, 1, 1, 0, 0, 0)
                            .unwrap()
                            .naive_utc(),
                        updated_at: Utc
                            .with_ymd_and_hms(2025, 1, 1, 0, 0, 0)
                            .unwrap()
                            .naive_utc(),
                    })
                })
            });

        mock_crew_switchboard_repo
            .expect_join()
            .returning(|_| Box::pin(async { Ok(()) }));

        let use_case = CrewSwitchboardUseCase::new(
            Arc::new(mock_crew_switchboard_repo),
            Arc::new(mock_quest_viewing_repo),
        );

        let result = use_case.join(1, 1).await;

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "The quest has adventures full"
        )
    }

    #[tokio::test]
    async fn test_leave_success() {
        let mut mock_crew_switchboard_repo = MockCrewSwitchboardRepository::new();
        let mut mock_quest_viewing_repo = MockQuestViewingRepository::new();

        mock_quest_viewing_repo
            .expect_adventurers_counting_by_quest_id()
            .returning(|_| Box::pin(async { Ok(2) }));

        mock_quest_viewing_repo
            .expect_view_details()
            .returning(|_| {
                Box::pin(async {
                    Ok(QuestEntity {
                        id: 1,
                        name: "test quest 1".to_string(),
                        description: Some("test quest description".to_string()),
                        status: QuestStatuses::Open.to_string(),
                        guild_commander_id: 1,
                        created_at: Utc
                            .with_ymd_and_hms(2025, 1, 1, 0, 0, 0)
                            .unwrap()
                            .naive_utc(),
                        updated_at: Utc
                            .with_ymd_and_hms(2025, 1, 1, 0, 0, 0)
                            .unwrap()
                            .naive_utc(),
                    })
                })
            });

        mock_crew_switchboard_repo
            .expect_leave()
            .returning(|_| Box::pin(async { Ok(()) }));

        let use_case = CrewSwitchboardUseCase::new(
            Arc::new(mock_crew_switchboard_repo),
            Arc::new(mock_quest_viewing_repo),
        );

        let result = use_case.leave(1, 1).await;

        assert!(result.is_ok())
    }

    #[tokio::test]
    async fn test_leave_fails_when_quest_is_not_open() {
        let mut mock_crew_switchboard_repo = MockCrewSwitchboardRepository::new();
        let mut mock_quest_viewing_repo = MockQuestViewingRepository::new();

        mock_quest_viewing_repo
            .expect_adventurers_counting_by_quest_id()
            .returning(|_| Box::pin(async { Ok(2) }));

        mock_quest_viewing_repo
            .expect_view_details()
            .returning(|_| {
                Box::pin(async {
                    Ok(QuestEntity {
                        id: 1,
                        name: "test quest 1".to_string(),
                        description: Some("test quest description".to_string()),
                        status: QuestStatuses::InJourney.to_string(),
                        guild_commander_id: 1,
                        created_at: Utc
                            .with_ymd_and_hms(2025, 1, 1, 0, 0, 0)
                            .unwrap()
                            .naive_utc(),
                        updated_at: Utc
                            .with_ymd_and_hms(2025, 1, 1, 0, 0, 0)
                            .unwrap()
                            .naive_utc(),
                    })
                })
            });

        mock_crew_switchboard_repo
            .expect_leave()
            .returning(|_| Box::pin(async { Ok(()) }));

        let use_case = CrewSwitchboardUseCase::new(
            Arc::new(mock_crew_switchboard_repo),
            Arc::new(mock_quest_viewing_repo),
        );

        let result = use_case.leave(1, 1).await;

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "The quest not leavable"
        )
    }
}
