// window.ApiClient
//   getPluginConfiguration()        -> readConfiguration
//   updatePluginConfiguration(body) -> writeConfiguration
//   getPublicSystemInfo()           -> systemInfo
//   getUsers()                      -> users
//   getVirtualFolders()             -> virtualFolders
// window.Dashboard
//   alert(text)                              -> notice
//   processPluginConfigurationUpdateResult() -> saveOutcome
//   showLoadingMsg()                         -> busy
//   hideLoadingMsg()                         -> idle
//
// every call posts {"call":<n>,"verb":"<name>","body":<json>} to the parent and
// resolves on {"call":<n>,"ok":<bool>,"value":<json>}; a plugin id the page
// supplies is dropped, because the host fixes the plugin from the frame that
// opened
(function () {
  var next = 0;
  var waiting = {};

  window.addEventListener('message', function (event) {
    var answer = event.data;
    if (typeof answer === 'string') {
      try {
        answer = JSON.parse(answer);
      } catch (e) {
        return;
      }
    }
    if (!answer || typeof answer.call !== 'number') {
      return;
    }
    var held = waiting[answer.call];
    if (!held) {
      return;
    }
    delete waiting[answer.call];
    if (answer.ok) {
      held.resolve(answer.value);
    } else {
      held.reject(new Error('refused'));
    }
  });

  function ask(verb, body) {
    next += 1;
    var call = next;
    return new Promise(function (resolve, reject) {
      waiting[call] = { resolve: resolve, reject: reject };
      window.parent.postMessage(
        JSON.stringify({ call: call, verb: verb, body: body === undefined ? null : body }),
        '*',
      );
    });
  }

  window.ApiClient = {
    getPluginConfiguration: function () {
      return ask('readConfiguration', null);
    },
    updatePluginConfiguration: function (a, b) {
      return ask('writeConfiguration', b === undefined ? a : b);
    },
    getPublicSystemInfo: function () {
      return ask('systemInfo', null);
    },
    getUsers: function () {
      return ask('users', null);
    },
    getVirtualFolders: function () {
      return ask('virtualFolders', null);
    },
  };

  window.Dashboard = {
    alert: function (text) {
      return ask('notice', typeof text === 'string' ? text : String(text));
    },
    processPluginConfigurationUpdateResult: function () {
      return ask('saveOutcome', null);
    },
    showLoadingMsg: function () {
      return ask('busy', null);
    },
    hideLoadingMsg: function () {
      return ask('idle', null);
    },
  };
})();
