package org.ws_benchmark.model;

import org.springframework.jdbc.core.RowMapper;

import java.sql.ResultSet;
import java.sql.SQLException;

public class RestMapper implements RowMapper<RestResult> {
    @Override
    public RestResult mapRow(ResultSet rs, int rowNum) throws SQLException {
        RestResult restResult = new RestResult();
        restResult.setId(rs.getInt("id"));
        restResult.setCode(rs.getString("code"));
        restResult.setName(rs.getString("name"));
        return restResult;
    }
}