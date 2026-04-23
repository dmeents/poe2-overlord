-- Omen items: store the activation effect description separately from the
-- generic currency description so the tooltip can render omens distinctly.
ALTER TABLE items ADD COLUMN omen_description TEXT;
