CREATE TABLE users
(
    id   SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL
);

INSERT INTO users (name)
VALUES ('出題者A');
INSERT INTO users (name)
VALUES ('回答者A');

CREATE TABLE questions
(
    id            SERIAL PRIMARY KEY,
    question      VARCHAR NOT NULL,
    answer        VARCHAR NOT NULL,
    questioner_id INTEGER NOT NULL
);

ALTER TABLE questions
    ADD CONSTRAINT fk_questioner_id FOREIGN KEY (questioner_id) REFERENCES users (id);
INSERT INTO questions (question, answer, questioner_id)
VALUES ('日本で一番高い山は？', '富士山', 1);

CREATE TABLE answers
(
    id           SERIAL PRIMARY KEY,
    users_id     INTEGER   NOT NULL,
    questions_id INTEGER   NOT NULL,
    answer       VARCHAR   NOT NULL,
    answered_at  TIMESTAMP NOT NULL
);

ALTER TABLE answers
    ADD CONSTRAINT fk_users_id FOREIGN KEY (users_id) REFERENCES users (id);
ALTER TABLE answers
    ADD CONSTRAINT fk_questions_id FOREIGN KEY (questions_id) REFERENCES questions (id);

INSERT INTO answers (users_id, questions_id, answer, answered_at)
VALUES (2, 1, '富士山', '2021-01-01 00:00:00');