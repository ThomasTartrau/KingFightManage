ALTER TABLE iam.token ADD COLUMN IF NOT EXISTS revoked_at timestamptz;