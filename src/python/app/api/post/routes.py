from flask import render_template, request, flash, redirect, url_for, current_app, abort
import json
import psycopg2
from psycopg2 import sql, errors
import uuid

from app.utilities.db_connection import db_connection
from app.posts.post_types import PostTypes
from app.authorization.authorize import authorize
from app.utilities.db_connection import db_connection

from app.api.post import post


@post.route("/api/post/<post_id>/edit")
@authorize(0)
@db_connection
def get_post_information(*args, post_id, connection=None, **kwargs):
	if connection is None:
		abort(500)

	postTypes = PostTypes()
	postTypesResult = postTypes.get_post_type_list(connection)

	item = {}
	try:
		cur = connection.cursor()

		cur.execute(
			sql.SQL("SELECT uuid, title, slug, content, js_file, css_file, post_status, publish_date, update_date, categories, tags FROM sloth_posts WHERE uuid = %s"), [post_id]
		)
		raw_item = cur.fetchone()
		cur.close()

		item = {
			"uuid": raw_item[0],
			"title": raw_item[1],
			"slug": raw_item[2],
			"content": raw_item[3],
			"jsFilePath": raw_item[4],
			"cssFilePath": raw_item[5],
			"postStatus": raw_item[6],
			"publishDate": raw_item[7],
			"updateDate": raw_item[8],
			"categories": raw_item[9],
			"tags": raw_item[10]
		}
	except Exception as e:
		print(e)
		connection.close()
		abort(500)

	connection.close()
	return json.dumps({ "postTypes": postTypesResult, "postInformation": item })

@post.route("/api/posts/<post_id>/new")
@authorize(0)
@db_connection
def prepare_new_post(*args, post_id, connection=None, **kwargs):
	if connection is None:
		abort(500)

	postTypes = PostTypes()
	postTypesResult = postTypes.get_post_type_list(connection)

	connection.close()
	return json.dumps({ "postTypes": postTypesResult, "newPostUuid": str(uuid.uuid4()) })


@post.route("/api/post/save", methods=['PUT'])
@authorize(0)
@db_connection
def save_post(*args, connection=None, **kwargs):
	if connection is None:
		abort(500)

	import pdb; pdb.set_trace()

	try:
		cur = connection.cursor()

		cur.execute(
			sql.SQL("UPDATE sloth_posts SET uuid = %s, title = %s, slug = %s, content = %s, js_file = %s, css_file = %s, post_status = %s, publish_date = %s, update_date = %s, categories = %s, tags = %s WHERE uuid = %s"), [post_id]
		)
		raw_item = cur.fetchone()
		cur.close()

		item = {
			"uuid": raw_item[0],
			"title": raw_item[1],
			"slug": raw_item[2],
			"content": raw_item[3],
			"jsFilePath": raw_item[4],
			"cssFilePath": raw_item[5],
			"postStatus": raw_item[6],
			"publishDate": raw_item[7],
			"updateDate": raw_item[8],
			"categories": raw_item[9],
			"tags": raw_item[10]
		}
	except Exception as e:
		print(e)
		connection.close()
		abort(500)


	connection.close()
	return json.dumps({ "postTypes": postTypesResult, "newPostUuid": str(uuid.uuid4()) })

@post.route("/api/post/create", methods=['POST'])
@authorize(0)
@db_connection
def create_new_post(*args, connection=None, **kwargs):
	if connection is None:
		abort(500)

	try:
		cur = connection.cursor()

		# TODO here must be insert instead
		cur.execute(
			sql.SQL("SELECT uuid, title, slug, content, js_file, css_file, post_status, publish_date, update_date, categories, tags FROM sloth_posts WHERE uuid = %s"), [post_id]
		)
		raw_item = cur.fetchone()
		cur.close()

		item = {
			"uuid": raw_item[0],
			"title": raw_item[1],
			"slug": raw_item[2],
			"content": raw_item[3],
			"jsFilePath": raw_item[4],
			"cssFilePath": raw_item[5],
			"postStatus": raw_item[6],
			"publishDate": raw_item[7],
			"updateDate": raw_item[8],
			"categories": raw_item[9],
			"tags": raw_item[10]
		}
	except Exception as e:
		print(e)
		connection.close()
		abort(500)


	connection.close()
	return json.dumps({ "postTypes": postTypesResult, "newPostUuid": str(uuid.uuid4()) })