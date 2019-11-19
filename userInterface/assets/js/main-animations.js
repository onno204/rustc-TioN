
function animation_loadingbutton_start(target = ".loading-button", inputfields = ".loading-input"){
  if (!$(target).hasClass("animations-loading-button-active")){
    $(target).addClass("animations-loading-button-active");
    $(target).append("<div class='animations-loading-button'><div></div><div></div><div></div><div></div></div>")
  }
  if (inputfields != undefined){
    if (!$(inputfields).hasClass("animations-loading-button-input-active")){
      $(inputfields).addClass("animations-loading-button-input-active");
      $(inputfields).prop('disabled', true);
    }
  }
}
function animation_loadingbutton_stop(target = ".loading-button", inputfields = ".loading-input"){
  if ($(target).hasClass("animations-loading-button-active")){
    $(target).removeClass("animations-loading-button-active");
    $(target).find('.animations-loading-button:first').remove();
  }
  if (inputfields != undefined){
    if ($(inputfields).hasClass("animations-loading-button-input-active")){
      $(inputfields).removeClass("animations-loading-button-input-active");
      $(inputfields).prop('disabled', false);
    }
  }
}
function animation_loadingbutton_isloading(target = ".loading-button", inputfields = ".loading-input"){
  if ($(target).hasClass("animations-loading-button-active")){
    return true;
  }else{
    return false;
  }
}
