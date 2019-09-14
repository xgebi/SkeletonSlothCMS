CREATE TABLE sloth_posts (
	uuid character varying(200) NOT NULL,
	slug character varying(200) UNIQUE,
	post_type character varying(200),
	author character varying(200) NULL,
	title character varying(220),
	content text,
	css_file text,
	js_file text,
	publish_date double precision,
	update_date double precision,
	post_status character varying(10),
	tags text[],
	categories text[],
	deleted boolean,

	PRIMARY KEY(uuid),
	CONSTRAINT post_type_fkey FOREIGN KEY(post_type)
		REFERENCES sloth_post_types (uuid) MATCH SIMPLE
		ON UPDATE NO ACTION
		ON DELETE NO ACTION,
	CONSTRAINT author_fkey FOREIGN KEY(author)
		REFERENCES sloth_users (uuid) MATCH SIMPLE
		ON UPDATE NO ACTION
		ON DELETE NO ACTION
);