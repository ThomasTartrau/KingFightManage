package fr.kingfight.kingManageAPI.utils;

public class Problem {
    private final String id;
    private final String title;
    private final String detail;

    public Problem(String id, String title, String detail) {
        this.id = id;
        this.title = title;
        this.detail = detail;
    }

    public String getId() {
        return id;
    }

    public String getTitle() {
        return title;
    }

    public String getDetail() {
        return detail;
    }
}
