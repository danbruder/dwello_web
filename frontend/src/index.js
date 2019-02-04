const {Elm} = require('./Main');
import './all.css'

var app = Elm.Main.init({
  flags: {
    api: process.env.ELM_APP_API || 'http://localhost:4444',
    token: localStorage.getItem('token') || '',
    googleApiKey: 'AIzaSyDyfTrLPc8DeeRpUY3QGaWTgKhtrJ2_Sxc'
  },
});

app.ports.setToken.subscribe(function(token) {
  localStorage.setItem('token', token);
});

app.ports.logout.subscribe(function(token) {
  localStorage.removeItem('token');
  window.location.href = '/login'
});
