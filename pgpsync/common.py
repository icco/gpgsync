import os, sys, re, platform
from PyQt4 import QtCore, QtGui

def alert(msg, icon=QtGui.QMessageBox.Warning):
    d = QtGui.QMessageBox()
    d.setWindowTitle('PGP Sync')
    d.setText(msg)
    d.setIcon(icon)
    d.exec_()

def valid_fp(fp):
    return re.match(r'^[a-f\d]{40}$', clean_fp(fp))

def clean_fp(fp):
    return fp.strip().replace(' ','').lower()

def clean_keyserver(keyserver):
    if '://' not in keyserver:
        return 'hkp://{s:0}'.format(keyserver)
    return keyserver

def get_image_path(filename):
    """
    if platform.system() == 'Linux':
        prefix = os.path.join(sys.prefix, 'share/pgpsync')
    elif platform.system() == 'Windows':
        prefix = os.path.dirname(os.path.dirname(os.path.abspath(inspect.getfile(inspect.currentframe()))))
    else:
        prefix = os.path.dirname(__file__)
    """
    # Commenting out the path logic until there's proper packaging
    prefix = os.path.join(os.path.dirname(os.path.abspath(sys.argv[0])), 'share')

    image_path = os.path.join(prefix, filename)
    return image_path

class LoadingAnimation(QtGui.QLabel):
    def __init__(self, parent=None):
        QtGui.QLabel.__init__(self, parent)

        self.setSizePolicy(QtGui.QSizePolicy.Expanding, QtGui.QSizePolicy.Expanding)
        self.setAlignment(QtCore.Qt.AlignCenter)
        self.setFixedWidth(16)
        self.setFixedHeight(16)

        self.movie = QtGui.QMovie(get_image_path('loading.gif'), QtCore.QByteArray(), self)
        self.movie.setCacheMode(QtGui.QMovie.CacheAll)
        self.movie.setSpeed(100)
        self.movie.start()
        self.setMovie(self.movie)

        self.show()