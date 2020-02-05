package org.ws_benchmark.model;

public class RestResult {
    private int id;
    private String code, name;
    // getter dan setter

    // id ---------------------
    public int getId() {
        return id;
    }
    public void setId(int id) {
        this.id = id;
    }

    // code ----------------------
    public String getCode() {
        return code;
    }
    public void setCode(String code) {
        this.code = code;
    }

    // name ---------------------------
    public String getName() {
        return name;
    }
    public void setName(String name) {
        this.name = name;
    }

}