<?php
$db = pg_connect("host=localhost port=5432 dbname=gag_eproc user=postgres");
$result = pg_query($db, "SELECT id, code, name from color");
$list = array();
while ($row = pg_fetch_assoc($result)) {
	array_push($list, $row);
}
echo json_encode($list);
?>
