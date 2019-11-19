
function register_checklogin(){
  if (animation_loadingbutton_isloading()) { return false; }
  animation_loadingbutton_start()
  var data = {};
  data['type'] = 'register';
  data['username'] = $("#loginForm :input[name='username']").val();
  data['password'] = $("#loginForm :input[name='password']").val();
  data['firstname'] = $("#loginForm :input[name='firstname']").val();
  data['lastname'] = $("#loginForm :input[name='lastname']").val();
  data['email'] = $("#loginForm :input[name='email']").val();
  data['phone'] = $("#loginForm :input[name='phone']").val();
  data['discord'] = $("#loginForm :input[name='discord']").val();
  sendPostRequest('register', data, function(data){
    animation_loadingbutton_stop();
    if (data.result.success){
      window.location = 'login'
    }
    console.log("data: ", data.result);
  });
}
