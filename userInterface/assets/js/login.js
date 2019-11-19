
function login_checklogin(){
  if (animation_loadingbutton_isloading()) { return false; }
  animation_loadingbutton_start()
  var data = {};
  data['type'] = 'login';
  data['username'] = $("#loginForm :input[name='username']").val();
  data['password'] = $("#loginForm :input[name='password']").val();
  sendPostRequest('login', data, function(data){
    animation_loadingbutton_stop();
    if (data.result.success){
      window.location = 'dashboard'
    }
    console.log("data: ", data.result);
  });
}
