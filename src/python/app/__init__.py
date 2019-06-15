# app/__init__.py
import os
from flask import Flask
#from flask_login import LoginManager
from flask_bcrypt import Bcrypt

#login_manager = LoginManager()
#login_manager.login_view = 'authentication.do_the_login'
#login_manager.session_protection = 'strong'
bcrypt = Bcrypt()


def create_app(config_type):  # dev, test, or prod

    app = Flask(__name__)
    configuration = os.path.join(os.getcwd(), 'config', config_type + '.py')
    app.config.from_pyfile(configuration)

    from app.entry import entry
    app.register_blueprint(entry)

    from app.wizard import wizard
    app.register_blueprint(wizard)
    bcrypt.init_app(app)

    return app