
var savedTables = {};
function vergelijking_database_save(id){
  if (animation_loadingbutton_isloading()) { return false; }
  animation_loadingbutton_start()
  vergelijking_gettable(id, function(table){
    var data = {};
    data['type'] = 'save';
    data['comparecontent'] = table;
    data['compareid'] = table['id'];
    sendPostRequest('compare', data, function(data){
      animation_loadingbutton_stop();
      if (data.result.success){
        alert('success');
      }else{ console.log("error: ", data.result); }
    });
  });
}
function vergelijking_database_get(id){
  var data = {};
  data['type'] = 'get';
  data['compareid'] = $('#' + id).attr("attr-id");
  sendPostRequest('compare', data, function(data){
    animation_loadingbutton_stop();
    if (data.result.success){
      vergelijking_rebuild_table(JSON.parse(data.result.result.content), id);
    }else{ console.log("error: ", data.result); }
  });
}

function vergelijking_settings_column(obj){

}

function vergelijking_add_column(obj){
  var id = $(obj).parent().parent().parent().attr('id');
  var index = $(obj).parent().index()-1;
  console.log("index: ", index);
  savedTables[id]['top'].splice(index, 0, savedTables[id]['top'][index]);
  savedTables[id]['values'].splice(index+1, 0, savedTables[id]['values'][index+1]);
  // Top
  var type = ""
  var children = $('#'+id).children().first().children();
  children.each(function(i){
    if ($(this).index() === index+1){
      type = $('#'+id).children().first().children().eq(index+1).attr('attr-type')
      var name = $('#'+id).children().first().children().eq(index+1).attr('attr-value')
      var tooltip =  $('#'+id).children().first().children().eq(index+1).attr('attr-tooltip')
      $(this).after("<div attr-type=\""+type+"\" attr-value=\""+name+"\" attr-tooltip=\""+tooltip+"\">"+name+"<span onclick=\"vergelijking_settings_column(this)\" class=\"settings fa fa-cog\"></span><span onclick=\"vergelijking_remove_column(this)\" class=\"remove fa fa-minus-circle\"></span><span onclick=\"vergelijking_add_column(this)\" class=\"add fa fa-times-circle\"></span></div>");
    }
  });
  // Values
  var children = $('#'+id).children();
  children.each(function(i){
    if ($(this).index() !== 0){
      var oldChilds = $(this).children();
      $(this).children().each(function(y){
        if ($(this).index() === index+1){
          var value = oldChilds.eq(index+1).attr('attr-value');
          console.log("value", value);
          $(this).after("<div attr-value=\""+value+"\">"+vergelijking_getHTMLitemFromValue(type, value)+"</div>");
        }
      });
    }
  });
  // vergelijking_rebuild_table(savedTables[id], id);
}
function vergelijking_remove_column(obj){
  var id = $(obj).parent().parent().parent().attr('id');
  var index = $(obj).parent().index()-1;
  console.log("index: ", );
  savedTables[id]['top'].splice(index, 1);
  savedTables[id]['values'].splice(index, 1);
  var removed = false;
  // Top
  var type = ""
  var children = $('#'+id).children().first().children();
  children.each(function(i){
    if ($(this).index() === index+1 && !removed){
      var targetelmt = $(this);
      targetelmt.addClass('removeAnimation');
      setTimeout(function() { targetelmt.remove(); }, 1000);
      removed = true;
    }
  });
  // Values
  var children = $('#'+id).children();
  children.each(function(i){
    if ($(this).index() !== 0){
      removed = false;
      var oldChilds = $(this).children();
      $(this).children().each(function(y){
        if ($(this).index() === index+1 && !removed){
          var targetelmt = $(this);
          targetelmt.addClass('removeAnimation');
          setTimeout(function() { targetelmt.remove(); }, 1000);
          removed = true;
        }
      });
    }
  });
  // vergelijking_rebuild_table(savedTables[id], id);
}

function vergelijking_getTableLeft(id, callback){
  var tableLeft = [];
  var children = $('#'+id).children();
  var count = children.length-1;
  children.each(function(i){
    if ($(this).index() !== 0){
      tableLeft.push({
        'src': $(this).find("img:first").attr('src'),
        'name': $(this).find("img:first").attr('alt')
      });
      if (i == count) { callback(tableLeft); }
    }
  });
}
function vergelijking_getTableTop(id, callback){
  var tableLayout = [];
  var children = $('#'+id).children().first().children();
  var count = children.length-1;
  children.each(function(i){
    if ($(this).index() !== 0){
      tableLayout.push({
      'type': $(this).attr('attr-type'),
      'name': $(this).attr('attr-value'),
      'tooltip': $(this).attr('attr-tooltip'),
      });
      if (i == count) { callback(tableLayout); }
    }
  });
}
function vergelijking_getTableValues(id, callback){
  var tableValues = [];
  var children = $('#'+id).children();
  var count = children.length-1;
  children.each(function(i){
    if ($(this).index() !== 0){
      $(this).children().each(function(y){
        if ($(this).index() !== 0){
          var index = $(this).parent().index();
          if (tableValues[y] == undefined){ tableValues[y] = []; }
          tableValues[y].push($(this).attr('attr-value'));
        }
      });
      if (i == count) { callback(tableValues); }
    }
  });
}
function vergelijking_gettable(id, callback){
  vergelijking_getTableValues(id, function(tableValues){
    vergelijking_getTableTop(id, function(tableTop){
      vergelijking_getTableLeft(id, function(tableLeft){
        var table = {
          'id': $('#' + id).attr("attr-id"),
          'values': tableValues,
          'top': tableTop,
          'left': tableLeft
        };
        console.log("table: ", table);
        callback(table, id);
      });
    });
  });
}
function vergelijking_rebuild_table(table, id){
  console.log("table: ", table);
  savedTables[id] = table;
  var tableValues = table.values;
  var tableTop = table.top;
  var tableLeft = table.left;
  var targetElmt = $('#'+id);
  targetElmt.html(" ")
  var outputBuilder = "<div class=\"v-table-top\"><div></div>";
  for (var i in tableTop) {
    var currentLayout = tableTop[i];
    outputBuilder += "<div attr-type=\""+currentLayout.type+"\" attr-value=\""+currentLayout.name+"\" attr-tooltip=\""+currentLayout.tooltip+"\">"+currentLayout.name+"<span onclick=\"vergelijking_settings_column(this)\" class=\"settings fa fa-cog\"></span><span onclick=\"vergelijking_remove_column(this)\" class=\"remove fa fa-minus-circle\"></span><span onclick=\"vergelijking_add_column(this)\" class=\"add fa fa-times-circle\"></span></div>";
  }
  outputBuilder += "</div>";
  targetElmt.append(outputBuilder)

  var outputBuilder = "";
  for (var i in tableLeft) {
    i = parseInt(i);
    var currentLeft = tableLeft[i];
    outputBuilder += "<div class=\"v-table-content\">";
    outputBuilder += "<div><img src=\"assets/img/banken/"+currentLeft.name+".png\" alt=\""+currentLeft.name+"\" /><label>"+currentLeft.name+"</label></div>";
    for (var y in tableTop) {
      y = parseInt(y);
      var currentValue = tableValues[y+1][i];
      outputBuilder += "<div attr-value=\""+currentValue+"\">"+vergelijking_getHTMLitemFromValue(tableTop[y].type, currentValue)+"</div>";
    }
    outputBuilder += "</div>";
  }
  targetElmt.append(outputBuilder);
  vergelijking_setup_listeners(id)
}

function vergelijking_setup_listeners(id){
  $(".v-table-editable").bind('click.'+id, function(){
    var xLoc = $(this).parent().index();
    var yLoc = $(this).parent().parent().index();
    var type = savedTables[id]['top'][xLoc-1]['type'];
    var value = $(this).parent().attr("attr-value");
    var newValue = vergelijking_change_value(type, value);
    savedTables[id]['values'][xLoc][yLoc] = newValue;
    $(this).parent().attr("attr-value", newValue);
    $(this).parent().html(vergelijking_getHTMLitemFromValue(type, newValue));
    console.log(xLoc, yLoc, type, value, newValue);
    $(".v-table-editable").unbind('click.'+id);
    vergelijking_setup_listeners(id);
  })
}
function vergelijking_change_value(type, value){
  switch (type){
    case "check":
      if (value === 'true'){ value = "false";
      }else if (value === 'false'){ value = "true"}
      return value;
      break;
    case "text":
      var input = window.prompt("Enter text", value);
      if (input == null){ return ""; }
      return input;
      break;
    default:
      return undefined;
  }
}


function vergelijking_getHTMLitemFromValue(currentItemType, currentItem){
  switch (currentItemType){
    case "check":
      if (currentItem === 'true'){
        return "<span class=\"fa fa-check v-table-editable\"></span>"
      }else{
        return "<span class=\"fa fa-times v-table-editable\"></span>"
      }
      break;
    case "text":
      return "<label class=\"v-table-editable\">"+currentItem.escape()+"</label>"
      break;
    default:
      return "<p>???</p>"
      break;
  }

}

// Remove HTML chars: https://stackoverflow.com/questions/5499078/fastest-method-to-escape-html-tags-as-html-entities
String.prototype.escape = function() {
  var tagsToReplace = {
    '&': '&amp;',
    '<': '&lt;',
    '>': '&gt;'
  };
  return this.replace(/[&<>]/g, function(tag) {
    return tagsToReplace[tag] || tag;
  });
};

// var a = "<abc>";
// var b = a.escape(); // "&lt;abc&gt;"
// JQeury function 'closest_descendent': https://stackoverflow.com/questions/8961770/similar-to-jquery-closest-but-traversing-descendants
(function($) {
  $.fn.closest_descendent = function(filter) {
    var $found = $(),
      $currentSet = this; // Current place
    while ($currentSet.length) {
      $found = $currentSet.filter(filter);
      if ($found.length) break;  // At least one match: break loop
      // Get all children of the current set
      $currentSet = $currentSet.children();
    }
    return $found.first(); // Return first match of the collection
  }
})(jQuery);
