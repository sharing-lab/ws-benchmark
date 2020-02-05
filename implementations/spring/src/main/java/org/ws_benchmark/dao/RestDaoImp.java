package org.ws_benchmark.dao;

import org.ws_benchmark.model.Rest;
import org.ws_benchmark.model.RestMapper;
import org.ws_benchmark.model.RestResult;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.jdbc.core.JdbcTemplate;
import org.springframework.stereotype.Repository;
import org.springframework.transaction.annotation.Transactional;

import java.util.List;
import java.util.Random;

@Transactional
@Repository
public class RestDaoImp implements RestDao {
    @Autowired
    private JdbcTemplate jdbcTemplate;

    @Override
    public List<RestResult> getAll() {
        String sql = "select id, code, name from color";
        List<RestResult> rest = jdbcTemplate.query(sql, new RestMapper());
        return rest;
    }

}