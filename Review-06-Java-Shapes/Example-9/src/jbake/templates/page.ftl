<#include "header.ftl">

    <div class="row h-100">
        <div class="col-sm-3 col-md-2 bg-light hidden-md-down" id="main-nav">
        <#include "menu.ftl">
        </div>

        <div class="col">
            <div class="mh-90">
                <div class="pageHeader">
                    <h1><#escape x as x?xml>${content.title}</#escape></h1>
                </div>
                <p><em>${content.date?string("dd MMMM yyyy")}</em></p>

                ${content.body}
            </div>
        </div>
    </div>


<#include "footer.ftl">
