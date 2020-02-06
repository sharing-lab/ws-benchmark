#!/usr/bin/perl
use Mojolicious::Lite;
use Mojo::Pg;

my $pg = Mojo::Pg->new('postgresql://postgres@localhost/test_db')
	->options({AutoCommit => 1, RaiseError => 1, AutoInactiveDestroy => 1})
	->max_connections(10);

helper pg => sub { $pg };

get '/color' => sub {
    my $c = shift;
    my $res = $c->pg->db->query('select id, code, name from color', sub {
        my ($db, $err, $results) = @_;
        $c->render(json => $results->hashes);
    });
    $c->render_later;
};

app->start;
