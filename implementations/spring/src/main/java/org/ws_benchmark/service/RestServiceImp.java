package org.ws_benchmark.service;

import org.ws_benchmark.dao.RestDao;
import org.ws_benchmark.model.Rest;
import org.ws_benchmark.model.RestResult;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;

import java.util.List;

@Service
public class RestServiceImp implements RestService{

    @Autowired
    private RestDao restDao;

    @Override
    public List<RestResult> getAll() {
        return restDao.getAll();
    }

}
