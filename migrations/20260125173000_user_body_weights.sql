-- Add user body weight tracking
CREATE TABLE user_body_weights (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    weight NUMERIC(10, 2) NOT NULL CHECK (weight > 0),
    weight_unit VARCHAR(20) NOT NULL DEFAULT 'kg',
    recorded_at TIMESTAMP NOT NULL DEFAULT NOW(),
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_user_body_weights_user_recorded_at
    ON user_body_weights(user_id, recorded_at DESC);
