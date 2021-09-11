from flask import request, redirect, make_response

from app.authorization.authorize import authorize_rest, authorize_web
from app.authorization.user import User, UserInfo
from app.toes.toes import render_toe_from_path
from app.toes.hooks import Hooks
import json
import os
from typing import Tuple

from app.login import login


@login.route("/login")
def show_login():
    return render_toe_from_path(
        path_to_templates=os.path.join(os.getcwd(), 'app', 'templates'),
        template="login.toe.html",
        data={
            "title": "Log in",
            "status": {},
            "redirect": request.args.get("redirect"),
        },
        hooks=Hooks()
    )


@login.route("/login/error")
def show_login_error():
    return render_toe_from_path(
        path_to_templates=os.path.join(os.getcwd(), 'app', 'templates'),
        template="login.toe.html",
        data={
            "title": "Log in",
            "status": {
                "error": True
            },
            "redirect": request.args.get("redirect")
        },
        hooks=Hooks()
    )


@login.route('/login/process', methods=["POST"])
def process_login(*args, connection=None, **kwargs):
    # get credentials
    username = request.form.get("username")
    password = request.form.get("password")
    if len(username) == 0 or len(password) == 0:
        return redirect("/login/error")
    # compare credentials with database
    user = User()
    info: UserInfo = user.login_user(username, password)

    # if good redirect to dashboard
    if info is not None:
        response = make_response(redirect('/dashboard' if request.args.get("redirect") is None else request.args.get("redirect")))
        response.set_cookie('sloth_session', f"{info.display_name}:{info.uuid}:{info.token}")
        return response
    return redirect("/login/error")


@login.route("/logout")
@authorize_web(0)
def logout(*args, permission_level, **kwargs):
    cookie = request.cookies.get('sloth_session')
    user = User(cookie[1], cookie[2])
    user.logout_user()

    response = make_response(redirect('/login'))
    response.set_cookie('sloth_session', "")
    return response


@login.route("/api/user/keep-logged-in", methods=["POST"])
@authorize_rest(0)
def keep_logged_in(*args, permission_level, **kwargs):
    return json.dumps({"loggedIn": True})


@login.route("/api/login")
def api_login() -> Tuple[str, int]:
    # get credentials
    data = json.loads(request.data)
    username = data.get("username")
    password = data.get("password")
    if len(username) == 0 or len(password) == 0:
        return json.dumps({"error": "name and password can't empty"}), 401
    # compare credentials with database
    user = User()
    info: UserInfo = user.login_user(username, password)

    # if good redirect to dashboard
    if info is not None:
        return info.to_json_string(), 200
    return json.dumps({"error": "Unable to login"}), 401


@login.route("/api/logout")
@authorize_rest(0)
def api_logout() -> Tuple[str, int]:
    data = json.loads(request.data)
    username = data.get("username")
    token = data.get("token")
    user = User(username, token)
    user.logout_user()
    return json.dumps({"status": "logged out"}), 200
