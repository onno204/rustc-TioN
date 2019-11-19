<?php
require_once "../config.php";
doesUserHavePermission("login", true, false, false);

require_once "../includes/header.php";
require_once "../includes/nav.php";

if(isset($_GET['logout'])){
  session_destroy();
  redirectToPage("login");
}else{
  if(isUserLoggedIn() == true){
    require_once "../includes/error.php";
    redirectToPage("dashboard");
    //startError(200, "Alread logged in", "You'r already logged in");
  }
}
?>
<div class="login-background">
  <img src="assets/img/background.jpg" alt="" />
</div>
<form class="loginForm" id="loginForm">
  <h1>Login</h1>
  <input class="loading-input" type="text" name="username" placeholder="Username">
  <input class="loading-input" type="password" name="password" placeholder="Password">
  <br />
  <div class="button loading-button" onclick="login_checklogin()">Inloggen</div>
  <div class="button nobackground loading-button button-inline" onclick="login_checklogin()">reset password</div>
  <div class="button nobackground loading-button button-inline" onclick="window.location = 'register'">register</div>
</form>
<?php
require_once "../includes/footer.php";
