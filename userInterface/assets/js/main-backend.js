

function sendPostRequest(action, data, callback){
  $.ajax({
    type: 'POST',
    url: 'backend/'+action+'.php',
    data: data,
    beforeSend: function (xhr) {
      xhr.setRequestHeader ("Authorization", "Basic " + authToken);
    },
    success: function(data){
      data['result'] = {'success': false};
      try{ data['result'] = JSON.parse(data.responseText)
      }catch(e){ console.log("error with callback",e);
      }finally{ callback(data); }
    },
    error: function(data){
      data['result'] = {'success': false, 'error': 'error_unknown'};
      try{ data['result'] = JSON.parse(data.responseText)
      }catch(e){ console.log("error with callback",e);
      }finally{ callback(data); }
    },
    dataType: "application/json",
  });
}
