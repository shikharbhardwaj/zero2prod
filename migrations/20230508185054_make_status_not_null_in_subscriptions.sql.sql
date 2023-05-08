BEGIN;
    -- Backfill `status` for existing rows.
    UPDATE subscriptions
        SET status = 'confirmed'
        WHERE status is NULL;

    -- Make `status` mandatory
    ALTER TABLE subscriptions ALTER COLUMN status SET NOT NULL;
COMMIT;
