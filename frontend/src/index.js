const { Elm } = require('./Main');

var app = Elm.Main.init({
  flags: {
    api: process.env.ELM_APP_API || 'http://localhost:4444',
    token: localStorage.getItem("token")
  },
});

app.ports.setToken.subscribe(function(token) {
  localStorage.setItem('token', token);
});
