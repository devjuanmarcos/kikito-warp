ALTER TABLE local_workspace_organizations
    ADD COLUMN accent_color TEXT NOT NULL DEFAULT 'blue';

ALTER TABLE local_workspace_organizations
    ADD COLUMN image_data BLOB;
