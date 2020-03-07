-- Your SQL goes here
CREATE TABLE posts (
  id bigint PRIMARY KEY auto_increment comment 'ID',
  title VARCHAR(256) NOT NULL comment '标题',
  body TEXT NOT NULL comment '内容',
  published BOOLEAN NOT NULL DEFAULT 0 comment '是否发布'
)