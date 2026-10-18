ALTER TABLE local_workspace_projects
ADD COLUMN last_opened_ts BIGINT NOT NULL DEFAULT 0;
