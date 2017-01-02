CREATE FUNCTION check_person_id() RETURNS trigger AS $check_person_id$
BEGIN
    IF NEW.person_type = 'student' THEN
        SELECT * INTO myrec FROM students WHERE id=NEW.person_id;
        IF NOT FOUND THEN
            RAISE EXCEPTION 'There is no student with id %', NEW.person_id;
        END IF;
    ELSE
        SELECT * INTO myrec FROM teachers WHERE id=NEW.person_id;
        IF NOT FOUND THEN
            RAISE EXCEPTION 'There is no teacher with id %', NEW.person_id;
        END IF;
    END IF;
    RETURN NEW;
END;
$check_person_id$ LANGUAGE plpgsql;

CREATE FUNCTION delete_lendings() RETURNS trigger AS $check_lendings$
BEGIN
    IF TG_TABLE_NAME = 'students' THEN
        DELETE FROM lendings WHERE person_type='student' AND person_id=OLD.id;
        DELETE FROM base_sets WHERE student_id=OLD.id;
    ELSE
        DELETE FROM lendings WHERE person_type='teacher' AND person_id=OLD.id;
    END IF;
    RETURN OLD;
END;
$check_lendings$ LANGUAGE plpgsql;

CREATE TRIGGER lendings_ins BEFORE INSERT OR UPDATE ON lendings FOR EACH ROW
EXECUTE PROCEDURE check_person_id();
CREATE TRIGGER del_student_lendings AFTER DELETE ON students FOR EACH ROW
EXECUTE PROCEDURE delete_lendings();
CREATE TRIGGER del_teacher_lendings AFTER DELETE ON teachers FOR EACH ROW
EXECUTE PROCEDURE delete_lendings();
